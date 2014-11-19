



fn main() {
    let mut first_agent = Agent {
        location: Point {
            x: 0,
            y: 0,
        },
        reserves: 100,
        chromosomes: vec!["ABCD1234".to_string(), "11100101".to_string()],
    };



    let mut some_resource = Resource {
        location: Point {
            x: 10,
            y: 10,
        },
        value: 100,
    };

    println!("\n")
    println!("I am first_agent my (x, y) is ({}, {})", first_agent.location.x, first_agent.location.y);
    println!("I have {} resources.", first_agent.reserves);
    println!("My chromosomes are {}.", first_agent.chromosomes);
    println!("\n")
    println!("I am some_resource my (x, y) is ({}, {})", some_resource.location.x, some_resource.location.y);
    println!("I have {} resources.", some_resource.value);

}

pub struct Point {
    pub y: int,
    pub x: int,
}

pub struct Agent  {
    pub location: Point,
    reserves: uint,
    chromosomes: Vec<String>,
}

pub struct Resource {
    pub location: Point,
    pub value: int,
}

trait Mobile {
    fn travel(&mut self, delta_x: int, delta_y: int);
}

impl Agent {
    fn new(x: int, y: int, resources: uint) -> Agent {
        Agent {
            location: Point {
                x: x,
                y: y,
            },
            reserves: resources,
            chromosomes: vec!["needs".to_string(), "default".to_string()]
        }
    }
}

impl Mobile for Agent {
    fn travel(&mut self, delta_x: int, delta_y: int) {
        self.location.x = *&self.location.x + delta_x;
        self.location.y = *&self.location.y + delta_y;
    }
}

mod test {
    #[test]
    fn test_sanity() {
        assert!(true)
    }

}
