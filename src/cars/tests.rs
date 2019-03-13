#[cfg(test)]
mod tests {
    use super::super::app::CarDao;
    use super::super::super::models::car::Car;
    use std::collections::HashMap;

    #[test]
    fn test_car_dao() {
        let mut car_dao = CarDao::new(HashMap::new());

        car_dao.create(Car {
            id: 5,
            make: String::from("foo"),
            model: String::from("bar"),
            color: String::from("baz"),
            year: 40
        }).unwrap();

        let car = car_dao.get(5).unwrap();
        assert_eq!(car.id, 5);
        assert_eq!(car.make, "foo");
        assert_eq!(car.model, "bar");
        assert_eq!(car.color, "baz");
        assert_eq!(car.year, 40);

        let cars = car_dao.get_all();
        assert_eq!(cars.count(), 1);
    }
}
