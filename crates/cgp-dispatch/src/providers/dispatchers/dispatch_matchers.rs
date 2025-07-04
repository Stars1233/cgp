use core::marker::PhantomData;

use cgp_core::prelude::*;
use cgp_handler::{
    Computer, ComputerComponent, Handler, HandlerComponent, TryComputer, TryComputerComponent,
};

pub struct DispatchMatchers<Handlers>(pub PhantomData<Handlers>);

#[cgp_provider]
impl<Context, Code, Input, Handlers, Output, Remainder> Computer<Context, Code, Input>
    for DispatchMatchers<Handlers>
where
    Handlers: DispatchComputer<Context, Code, Input, Output = Output, Remainder = Remainder>,
{
    type Output = Result<Output, Remainder>;

    fn compute(_context: &Context, code: PhantomData<Code>, input: Input) -> Self::Output {
        Handlers::compute(_context, code, input)
    }
}

#[cgp_provider]
impl<Context, Code, Input, Handlers, Output, Remainder> TryComputer<Context, Code, Input>
    for DispatchMatchers<Handlers>
where
    Context: HasErrorType,
    Handlers: TryDispatchComputer<Context, Code, Input, Output = Output, Remainder = Remainder>,
{
    type Output = Result<Output, Remainder>;

    fn try_compute(
        _context: &Context,
        code: PhantomData<Code>,
        input: Input,
    ) -> Result<Self::Output, Context::Error> {
        Handlers::try_compute(_context, code, input)
    }
}

#[cgp_provider]
impl<Context, Code: Send, Input: Send, Handlers, Output: Send, Remainder: Send>
    Handler<Context, Code, Input> for DispatchMatchers<Handlers>
where
    Context: HasAsyncErrorType,
    Handlers: DispatchHandler<Context, Code, Input, Output = Output, Remainder = Remainder>,
{
    type Output = Result<Output, Remainder>;

    async fn handle(
        _context: &Context,
        code: PhantomData<Code>,
        input: Input,
    ) -> Result<Self::Output, Context::Error> {
        Handlers::handle(_context, code, input).await
    }
}

trait DispatchComputer<Context, Code, Input> {
    type Output;

    type Remainder;

    fn compute(
        context: &Context,
        tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Self::Output, Self::Remainder>;
}

trait TryDispatchComputer<Context, Code, Input>
where
    Context: HasErrorType,
{
    type Output;

    type Remainder;

    fn try_compute(
        context: &Context,
        tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Result<Self::Output, Self::Remainder>, Context::Error>;
}

impl<
        Context,
        Code,
        Input,
        CurrentHandler,
        NextHandler,
        RestHandlers,
        Output,
        RemainderA,
        RemainderB,
    > DispatchComputer<Context, Code, Input>
    for Cons<CurrentHandler, Cons<NextHandler, RestHandlers>>
where
    CurrentHandler: Computer<Context, Code, Input, Output = Result<Output, RemainderA>>,
    Cons<NextHandler, RestHandlers>:
        DispatchComputer<Context, Code, RemainderA, Output = Output, Remainder = RemainderB>,
{
    type Output = Output;

    type Remainder = RemainderB;

    fn compute(
        context: &Context,
        tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Self::Output, Self::Remainder> {
        let res = CurrentHandler::compute(context, tag, input);

        match res {
            Ok(output) => Ok(output),
            Err(remainder) => Cons::compute(context, tag, remainder),
        }
    }
}

impl<
        Context,
        Code,
        Input,
        CurrentHandler,
        NextHandler,
        RestHandlers,
        Output,
        RemainderA,
        RemainderB,
    > TryDispatchComputer<Context, Code, Input>
    for Cons<CurrentHandler, Cons<NextHandler, RestHandlers>>
where
    Context: HasErrorType,
    CurrentHandler: TryComputer<Context, Code, Input, Output = Result<Output, RemainderA>>,
    Cons<NextHandler, RestHandlers>:
        TryDispatchComputer<Context, Code, RemainderA, Output = Output, Remainder = RemainderB>,
{
    type Output = Output;

    type Remainder = RemainderB;

    fn try_compute(
        context: &Context,
        tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Result<Self::Output, Self::Remainder>, Context::Error> {
        let res = CurrentHandler::try_compute(context, tag, input)?;

        match res {
            Ok(output) => Ok(Ok(output)),
            Err(remainder) => Cons::try_compute(context, tag, remainder),
        }
    }
}

impl<Context, Code, Input, Handler, Remainder, Output> DispatchComputer<Context, Code, Input>
    for Cons<Handler, Nil>
where
    Handler: Computer<Context, Code, Input, Output = Result<Output, Remainder>>,
{
    type Output = Output;

    type Remainder = Remainder;

    fn compute(
        context: &Context,
        tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Self::Output, Self::Remainder> {
        Handler::compute(context, tag, input)
    }
}

impl<Context, Code, Input, Handler, Remainder, Output> TryDispatchComputer<Context, Code, Input>
    for Cons<Handler, Nil>
where
    Context: HasErrorType,
    Handler: TryComputer<Context, Code, Input, Output = Result<Output, Remainder>>,
{
    type Output = Output;

    type Remainder = Remainder;

    fn try_compute(
        context: &Context,
        tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Result<Self::Output, Self::Remainder>, Context::Error> {
        Handler::try_compute(context, tag, input)
    }
}

#[async_trait]
trait DispatchHandler<Context, Code, Input>
where
    Context: HasErrorType,
{
    type Output;

    type Remainder;

    async fn handle(
        context: &Context,
        tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Result<Self::Output, Self::Remainder>, Context::Error>;
}

impl<
        Context,
        Code: Send,
        Input: Send,
        CurrentHandler,
        NextHandler,
        RestHandlers,
        Output: Send,
        RemainderA: Send,
        RemainderB: Send,
    > DispatchHandler<Context, Code, Input>
    for Cons<CurrentHandler, Cons<NextHandler, RestHandlers>>
where
    Context: HasAsyncErrorType,
    CurrentHandler: Handler<Context, Code, Input, Output = Result<Output, RemainderA>>,
    Cons<NextHandler, RestHandlers>:
        DispatchHandler<Context, Code, RemainderA, Output = Output, Remainder = RemainderB>,
{
    type Output = Output;

    type Remainder = RemainderB;

    async fn handle(
        context: &Context,
        tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Result<Self::Output, Self::Remainder>, Context::Error> {
        let res = CurrentHandler::handle(context, tag, input).await?;

        match res {
            Ok(output) => Ok(Ok(output)),
            Err(remainder) => Cons::handle(context, tag, remainder).await,
        }
    }
}

impl<Context, Code: Send, Input: Send, CurrentHandler, Remainder: Send, Output: Send>
    DispatchHandler<Context, Code, Input> for Cons<CurrentHandler, Nil>
where
    Context: HasAsyncErrorType,
    CurrentHandler: Handler<Context, Code, Input, Output = Result<Output, Remainder>>,
{
    type Output = Output;

    type Remainder = Remainder;

    async fn handle(
        context: &Context,
        tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Result<Self::Output, Self::Remainder>, Context::Error> {
        CurrentHandler::handle(context, tag, input).await
    }
}
