use cgp_core::prelude::*;

use crate::{
    Computer, ComputerComponent, Handler, HandlerComponent, TryComputer, TryComputerComponent,
};

pub struct ReturnInput;

#[cgp_provider]
impl<Context, Code, Input> Computer<Context, Code, Input> for ReturnInput {
    type Output = Input;

    fn compute(_context: &Context, _code: PhantomData<Code>, input: Input) -> Self::Output {
        input
    }
}

#[cgp_provider]
impl<Context, Code, Input> TryComputer<Context, Code, Input> for ReturnInput
where
    Context: HasErrorType,
{
    type Output = Input;

    fn try_compute(
        _context: &Context,
        _code: PhantomData<Code>,
        input: Input,
    ) -> Result<Self::Output, Context::Error> {
        Ok(input)
    }
}

#[cgp_provider]
impl<Context, Code: Send, Input: Send> Handler<Context, Code, Input> for ReturnInput
where
    Context: HasAsyncErrorType,
{
    type Output = Input;

    async fn handle(
        _context: &Context,
        _code: PhantomData<Code>,
        input: Input,
    ) -> Result<Self::Output, Context::Error> {
        Ok(input)
    }
}
