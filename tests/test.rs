use macros::inherit;
use objkt::class;

class!(Person, 
    {
        let name: String;
        let last_name: String;
        fn say_hello() {
            println!("Hello");
        }
        fn live() {
            println!("Living...")
        }
    },
);

inherit!(Worker, Person,
    {
        let job: String;
        fn work() {
            println!("Working...");
        }
    }
);

#[test]
fn test() {
    Person::live();
    Worker::live();
    Worker::work();
}