use dircmp::Comparison;

fn main() {
    let cmp = Comparison::default();
    let res = cmp
        .compare("dircmp/examples", "dircmp/src")
        .expect("should compare");
    println!("{:?}", res);
    // Diff { right: "dircmp/src", left: "dircmp/examples", similar: [], changed: [], missing_right: ["dircmp/examples/demo.rs"], missing_left: ["dircmp/src/lib.rs"], different_type: [] }
}
