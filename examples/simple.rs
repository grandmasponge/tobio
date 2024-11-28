

fn main() {
    let mut executor = tobio::executor::Executor::new();

    executor.spawn(async {
        let apples = "apples";
        println!("{apples}");
    });

    executor.run();
}