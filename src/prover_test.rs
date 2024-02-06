#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::time::Duration;

    use futures::future::BoxFuture;
    use futures::FutureExt;

    use zmq_rust_example::generated::prover::{
        ProverHandlers, ProverServer, TlsProof, ValidationResult,
    };

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // multi thread test
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn request_server_test() {
        println!("Starting server");
        struct ProverHandlersImpl {}
        impl ProverHandlers for ProverHandlersImpl {
            fn validate(&self, input: TlsProof) -> BoxFuture<Result<ValidationResult, ()>> {
                // get value of input.data, coerce to a number, delay the amount of time in ms
                let time = input.data.parse::<u64>().unwrap();
                // return a ValidationResult of ok if string says ok, not good otherwise
                let ok = input.id == "ok";
                async move {
                    tokio::time::sleep(Duration::from_millis(time)).await;
                    let id = format!("ok after {time}ms");
                    println!("id: {}", id);
                    Ok(ValidationResult { id, ok })
                }
                .boxed()
            }
        }
        let reply_handlers = Arc::new(ProverHandlersImpl {});

        let prove_server = ProverServer::new(
            "test_pub".to_string(),
            "test_req".to_string(),
            reply_handlers,
        );
        prove_server.start_listening();

        println!("stopped?");
    }

    // single thread test
    #[tokio::test(flavor = "current_thread")]
    async fn publish_test() {
        println!("Starting publisher");
        struct EmptyProverHandlersImpl {}
        impl ProverHandlers for EmptyProverHandlersImpl {}
        let reply_handlers = Arc::new(EmptyProverHandlersImpl {});
        let mut prove_server = ProverServer::new(
            "test_pub".to_string(),
            "test_req".to_string(),
            reply_handlers,
        );

        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            println!("Publishing message...");

            prove_server
                .publish_to_proofs(TlsProof {
                    id: "ok".to_string(),
                    data: "1000".to_string(),
                })
                .expect("Failed to publish");
        }
    }
}
