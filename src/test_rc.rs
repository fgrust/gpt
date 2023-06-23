#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};

    #[test]
    fn tests_reference_counter() {
        let x = Rc::new(RefCell::new(50));
        let y = Rc::clone(&x);
        let z = Rc::clone(&x);

        *x.borrow_mut() = 70;

        dbg!(x.borrow());
        dbg!(y.borrow());
        dbg!(z.borrow());

        #[derive(Debug)]
        struct House {
            address_number: u16,
            street: String,
            furniture: RefCell<Vec<Rc<Furniture>>>,
        }

        #[derive(Debug)]
        struct Furniture {
            id: String,
            description: String,
            house: Weak<House>,
        }

        let house_1 = Rc::new(House {
            address_number: 123,
            street: "coding avenue".to_string(),
            furniture: RefCell::new(vec![]),
        });

        let table = Rc::new(Furniture {
            id: "table1".to_string(),
            description: "kitchen table".to_string(),
            house: Rc::downgrade(&house_1),
        });

        let desk = Rc::new(Furniture {
            id: "desk1".to_string(),
            description: "office desk".to_string(),
            house: Rc::downgrade(&house_1),
        });

        house_1.furniture.borrow_mut().push(Rc::clone(&table));
        house_1.furniture.borrow_mut().push(Rc::clone(&desk));

        dbg!(house_1);
    }
}
