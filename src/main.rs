#![feature(proc_macro)]
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
use iron::mime::Mime;

use iron_pipeline::prelude::*;
use iron_pipeline::{Middleware, PipelineNext};

use router::Router;

mod handler;
mod json_object;


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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct OverviewData {
	vcpu_total: u32,
	vcpu_in_use: u32,
	ram_total: u32,
	ram_in_use: u32,
}

fn overview(_: &mut Request) -> IronResult<Response> {
	let data = OverviewData {
		vcpu_total: 8,
		vcpu_in_use: 4,
		ram_total: 32,
		ram_in_use: 4,
	};

	let m: Mime = "application/json".parse().unwrap();
	Ok(Response::with((status::Ok, m, serde_json::to_string(&data).unwrap())))
}

fn main() {
    let mut pipeline = Pipeline::new();

    let mut router = Router::new();
    router.get("/overview", overview, "overview");
    router.post("/token", handler::identity::login, "token");

    pipeline.add(Logging);
    pipeline.add(NotFound);
    pipeline.add(router);

    Iron::new(pipeline).http("0.0.0.0:3000").unwrap();
}
