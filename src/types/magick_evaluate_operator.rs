use crate::bindings;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum MagickEvaluateOperator {
    Undefined           = bindings::MagickEvaluateOperator_UndefinedEvaluateOperator,
    Abs                 = bindings::MagickEvaluateOperator_AbsEvaluateOperator,
    Add                 = bindings::MagickEvaluateOperator_AddEvaluateOperator,
    AddModulus          = bindings::MagickEvaluateOperator_AddModulusEvaluateOperator,
    And                 = bindings::MagickEvaluateOperator_AndEvaluateOperator,
    Cosine              = bindings::MagickEvaluateOperator_CosineEvaluateOperator,
    Divide              = bindings::MagickEvaluateOperator_DivideEvaluateOperator,
    Exponential         = bindings::MagickEvaluateOperator_ExponentialEvaluateOperator,
    GaussianNoise       = bindings::MagickEvaluateOperator_GaussianNoiseEvaluateOperator,
    ImpulseNoise        = bindings::MagickEvaluateOperator_ImpulseNoiseEvaluateOperator,
    LaplacianNoise      = bindings::MagickEvaluateOperator_LaplacianNoiseEvaluateOperator,
    LeftShift           = bindings::MagickEvaluateOperator_LeftShiftEvaluateOperator,
    Log                 = bindings::MagickEvaluateOperator_LogEvaluateOperator,
    Max                 = bindings::MagickEvaluateOperator_MaxEvaluateOperator,
    Mean                = bindings::MagickEvaluateOperator_MeanEvaluateOperator,
    Median              = bindings::MagickEvaluateOperator_MedianEvaluateOperator,
    Min                 = bindings::MagickEvaluateOperator_MinEvaluateOperator,
    MultiplicativeNoise = bindings::MagickEvaluateOperator_MultiplicativeNoiseEvaluateOperator,
    Multiply            = bindings::MagickEvaluateOperator_MultiplyEvaluateOperator,
    Or                  = bindings::MagickEvaluateOperator_OrEvaluateOperator,
    PoissonNoise        = bindings::MagickEvaluateOperator_PoissonNoiseEvaluateOperator,
    Pow                 = bindings::MagickEvaluateOperator_PowEvaluateOperator,
    RightShift          = bindings::MagickEvaluateOperator_RightShiftEvaluateOperator,
    RootMeanSquare      = bindings::MagickEvaluateOperator_RootMeanSquareEvaluateOperator,
    Set                 = bindings::MagickEvaluateOperator_SetEvaluateOperator,
    Sine                = bindings::MagickEvaluateOperator_SineEvaluateOperator,
    Subtract            = bindings::MagickEvaluateOperator_SubtractEvaluateOperator,
    Sum                 = bindings::MagickEvaluateOperator_SumEvaluateOperator,
    ThresholdBlack      = bindings::MagickEvaluateOperator_ThresholdBlackEvaluateOperator,
    Threshold           = bindings::MagickEvaluateOperator_ThresholdEvaluateOperator,
    ThresholdWhite      = bindings::MagickEvaluateOperator_ThresholdWhiteEvaluateOperator,
    UniformNoise        = bindings::MagickEvaluateOperator_UniformNoiseEvaluateOperator,
    Xor                 = bindings::MagickEvaluateOperator_XorEvaluateOperator,
    InverseLog          = bindings::MagickEvaluateOperator_InverseLogEvaluateOperator,
}

impl Default for MagickEvaluateOperator {
    fn default() -> Self {
        return MagickEvaluateOperator::Undefined;
    }
}

impl From<MagickEvaluateOperator> for bindings::MagickEvaluateOperator {
    fn from(value: MagickEvaluateOperator) -> Self {
        return value as bindings::MagickEvaluateOperator;
    }
}
