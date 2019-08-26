use crate::internal::*;
use ndarray::prelude::*;

#[derive(Debug, Clone, new, Default)]
pub struct Slice<D: DimLike + ToDim> {
    axes: Vec<usize>,
    starts: Vec<D>,
    ends: Vec<D>,
}

impl<D: DimLike + ToDim> Slice<D> {
    fn eval_t<T: Datum>(&self, input: Arc<Tensor>) -> TractResult<Arc<Tensor>> {
        let mut input = input.to_array_view::<T>()?;
        for (axis, b, e) in itertools::izip!(&self.axes, &self.starts, &self.ends) {
            input.slice_axis_inplace(
                Axis(*axis),
                ::ndarray::Slice::from((b.to_integer()?)..(e.to_integer()?)),
            );
        }
        if input.len() == 0 {
            unsafe {
                Ok(Tensor::from_raw::<T>(input.shape(), &[])?.into())
            }
        } else {
            Ok(Tensor::from(input.to_owned()).into())
        }
    }
}

impl<D: DimLike + ToDim> Op for Slice<D> {
    fn name(&self) -> Cow<str> {
        "Slice".into()
    }

    fn translation_invariants(
        &self,
        model: &TypedModel,
        node: &TypedNode,
    ) -> TractResult<Vec<TranslationInvariant>> {
        let fact = model.outlet_fact(node.inputs[0])?;
        let axes = (0..fact.shape.rank())
            .filter(|ax| !self.axes.contains(ax))
            .map(|axis| TranslationInvariant { axis, period: 1 })
            .collect();
        Ok(axes)
    }
}

impl<D: DimLike + ToDim> StatelessOp for Slice<D> {
    /// Evaluates the operation given the input tensors.
    fn eval(&self, mut inputs: TVec<Arc<Tensor>>) -> TractResult<TVec<Arc<Tensor>>> {
        let input = args_1!(inputs);
        Ok(tvec!(dispatch_datum!(Self::eval_t(input.datum_type())(self, input))?))
    }
}

impl<D: DimLike + ToDim> InferenceRulesOp for Slice<D> {
    fn rules<'r, 'p: 'r, 's: 'r>(
        &'s self,
        s: &mut Solver<'r>,
        inputs: &'p [TensorProxy],
        outputs: &'p [TensorProxy],
    ) -> InferenceResult {
        check_input_arity(&inputs, 1)?;
        check_output_arity(&outputs, 1)?;
        s.equals(&inputs[0].rank, &outputs[0].rank)?;
        s.equals(&inputs[0].datum_type, &outputs[0].datum_type)?;
        s.given(&inputs[0].rank, move |s, rank| {
            (0..(rank as usize)).try_for_each(move |axis| {
                if let Some(pos) = self.axes.iter().position(|i| i == &axis) {
                    s.equals(
                        &outputs[0].shape[axis],
                        (self.ends[pos].clone() - &self.starts[pos]).to_dim(),
                    )
                } else {
                    s.equals(&outputs[0].shape[axis], &inputs[0].shape[axis])
                }
            })
        })?;
        Ok(())
    }

    inference_op_as_op!();
    to_typed!();
}

impl<D: DimLike + ToDim> TypedOp for Slice<D> {
    typed_op_as_op!();

    fn output_facts(
        &self,
        inputs: &[&TypedTensorInfo],
    ) -> TractResult<TVec<TypedTensorInfo>> {
        let mut fact = inputs[0].clone();
        for (axis, b, e) in itertools::izip!(&self.axes, &self.starts, &self.ends) {
            fact.shape.set_dim(*axis, (e.clone() - b).to_dim())?;
        }
        Ok(tvec!(fact))
    }
}
