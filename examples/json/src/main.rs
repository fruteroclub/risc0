// Copyright 2024 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use serde::{Deserialize, Serialize};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

use json_core::Outputs;
use json_methods::SEARCH_JSON_ELF;
use risc0_zkvm::{default_prover, ExecutorEnv, sha::Digest};

use std::sync::Mutex;
use lazy_static::lazy_static;

#[derive(Serialize, Deserialize)]
struct Proof {
    hash: Digest,
}

async fn create_proof(item: web::Json<Proof>) -> impl Responder {
    let outputs = search_json(data);
    HttpResponse::Created().json(*outputs.hash)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/proof", web::post().to(create_proof))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

/*fn main() {
    let data = include_str!("../res/example.json");
    let outputs = search_json(data);
    println!();
    println!("  {:?}", outputs.hash);
}*/

fn search_json(data: &str) -> Outputs {
    let env = ExecutorEnv::builder()
        .write(&data)
        .unwrap()
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove(env, SEARCH_JSON_ELF).unwrap().receipt;

    receipt.journal.decode().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn main() {
        let data = include_str!("../res/example.json");
        let outputs = super::search_json(data);
        println!("The proof for you config file is {:?}", outputs.hash);
    }
}
