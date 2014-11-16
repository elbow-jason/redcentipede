



fn main() {
    let first_agent = Agent {
        x: 0,
        y: 0,
        reserves: 100,
        chromosomes: vec!["ABCD1234".to_string(), "11100101".to_string()],
    };

    let some_resource = Resource {
        x: 10,
        y: 10,
        value: 100,
    };

    println!("\n")
    println!("I am first_agent my (x, y) is ({}, {})", first_agent.x, first_agent.y);
    println!("I have {} resources.", first_agent.reserves);
    println!("My chromosomes are {}.", first_agent.chromosomes);
    println!("\n")
    println!("I am some_resource my (x, y) is ({}, {})", some_resource.x, some_resource.y);
    println!("I have {} resources.", some_resource.value);
}

struct Agent  {
    x: int,
    y: int,
    reserves: int,
    chromosomes: Vec<String>,
}

struct Resource {
    x: int,
    y: int,
    value: int,
}




