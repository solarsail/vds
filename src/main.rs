#![feature(proc_macro)]
#![feature(custom_attribute)]
extern crate iron;
extern crate iron_pipeline;
extern crate router;
extern crate hyper;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use iron::prelude::*;
use iron::status;

use iron_pipeline::prelude::*;
use iron_pipeline::{Middleware, PipelineNext};

use router::Router;

mod handler;
mod json_object;
mod http_object;


struct Logging;
impl Middleware for Logging {
	fn process(&self, req: &mut Request, next: PipelineNext) -> IronResult<Response> {
		println!("{:?}", req);
		let res = next.process(req);
		println!("{:?}", res);
		res
	}
}

struct NotFound;
impl Middleware for NotFound {
    fn process(&self, req: &mut Request, next: PipelineNext) -> IronResult<Response> {
    	let res = next.process(req);
		match res {
			Err(IronError {error, response}) => {
				if error.description() == "No Route" {
					Ok(Response::with((status::NotFound, "404 Not Found")))
				} else {
					Ok(response.set(error.description()))
				}
			}
			res => res
		}
	}
}

fn main() {
    let mut pipeline = Pipeline::new();

    let mut router = Router::new();
    router.get("/usage/:project_id", handler::compute::usage, "usage");
    router.post("/token", handler::identity::token, "token");

    pipeline.add(Logging);
    pipeline.add(NotFound);
    pipeline.add(router);

    Iron::new(pipeline).http("0.0.0.0:3000").unwrap();
}
