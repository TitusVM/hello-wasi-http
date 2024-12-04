mod bindings;

pub use bindings::wasi::http::types::{
    Fields, IncomingRequest, OutgoingBody, OutgoingResponse, ResponseOutparam,
};

use bindings::component::rpn::types::{Engine, Operation};

struct Component;

bindings::export!(Component with_types_in bindings);

impl bindings::exports::wasi::http::incoming_handler::Guest for Component {
    fn handle(_request: IncomingRequest, outparam: ResponseOutparam) {
        let hdrs = Fields::new();
        let resp = OutgoingResponse::new(hdrs);
        let body = resp.body().expect("outgoing response");

        ResponseOutparam::set(outparam, Ok(resp));

        let calc = Engine::new();
        calc.push_operand(1);
        calc.push_operand(2);
        calc.push_operation(Operation::Add);
        let sum: u32 = calc.execute();

        let out = body.write().expect("outgoing stream");
        out.blocking_write_and_flush(format!("[SUM] {:?}\n", sum).as_bytes())
            .expect("writing response");

        drop(out);
        OutgoingBody::finish(body, None).unwrap();
    }
}
