use std::collections::HashSet;

fn main() {
    // Set up vec of integers from 0 to 10
    let s: Vec<Element> = (0..=10).map(|i| Element(i)).collect();

    // Decide a test element to check order/generator status for
    let test_element = Element(2);

    // Create group operation as a closure
    let mul_closure = |e1: &Element, e2: &Element| 
        Element::multiplication(&e1, &e2, s.len() as u32);

    // Create group
    let group = Group::new(&s, mul_closure);

    println!("Group has order: {}", group.order());

    let el_is_generator = group.has_generator(&test_element);

    println!(
        "Checking if element {} is is generator for g: {}",
        test_element.0, el_is_generator
    );

    // Get element order for test element
    let element_order = group.element_order(&test_element);

    if let Some(order) = element_order {
        println!(
            "Element {:?} has order: {}",
            test_element, order
        );
    } else {
        println!("Unable to determine order of element {}", test_element.0);
    }
}

struct Group<F>
where
    F: Fn(&Element, &Element) -> Element,
{
    pub set: HashSet<Element>,
    pub operation: F,
}

impl<F> Group<F>
where
    F: Fn(&Element, &Element) -> Element,
{
    fn new(elements: &[Element], operation: F) -> Self {
        let mut set = HashSet::new();

        for el in elements {
            set.insert(el.clone());
        }

        Self { set, operation }
    }

    pub fn order(&self) -> usize {
        self.set.len()
    }

    fn element_order(&self, e: &Element) -> Option<usize> {
        if !self.set.contains(e) {
            return None;
        }

        let mut el = e.clone();
        let mut order = 0;
        let mut seen_elements = HashSet::new();

        loop {
            // println!("====================================");
            // println!("order: {:#?}", order);
            // println!("el: {:#?}", el);

            // End of loop, unit element found
            if el.0 == 1 {
                break;
            }

            // If we see an element twice before finding 1, then the original
            // element is not an generator.
            if seen_elements.contains(&el) {
                return None;
            }

            seen_elements.insert(el.clone());
            el = self.apply_operation(&el, &e);
            order += 1;
        }

        Some(order)
    }

    fn apply_operation(&self, a: &Element, b: &Element) -> Element {
        (self.operation)(a, b)
    }

    fn has_generator(&self, g: &Element) -> bool {
        // Clone set to enable modification
        let mut s = self.set.clone();
        let mut element = g.clone();
        let mut i = 0;

        while i < self.set.len() {
            // Check for unit element
            if g.0 == 1 {
                break;
            }

            let next = self.apply_operation(&element, g);

            s.remove(&next);

            element = next;
            i += 1;
        }

        // There should only be one element left in the set if g is a generator
        s.len() == 1
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct Element(u32);

impl Element {
    fn multiplication(a: &Element, b: &Element, modulo: u32) -> Self {
        let product = (a.0 * b.0) % modulo;
        // println!("Multiplying {} times {} mod {}, got {}",a.0, b.0, modulo, product);
        Element(product)
    }
}
