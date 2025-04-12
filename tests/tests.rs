
use howlast::howlast;

#[test]
fn it_works() {
    howlast!(step_duration => {
        let x = 1 + 1;
        std::thread::sleep(std::time::Duration::from_secs(1));
        x
    });
    print!("{:?}", step_duration);

    howlast!(step_duration, result => {
        let x = 1 + 1;
        std::thread::sleep(std::time::Duration::from_secs(1));
        x
    });
    print!("{:?} {:?}", step_duration, result);
}
