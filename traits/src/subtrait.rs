trait Foo {
    fn call(&self) {
        println!("in Foo")
    }
}

// trait SpecialFoo : Foo is syntactic sugar for:
trait SpecialFoo
where
    Self: Foo,
{
    fn call(&self) {
        println!("in SpecialFoo")
    }
}

struct A;

impl Foo for A {}
impl SpecialFoo for A {}

#[test]
fn test_special_foo() {
    let x = A;
    Foo::call(&x);
    <A as Foo>::call(&x);

    SpecialFoo::call(&x);
    <A as SpecialFoo>::call(&x)
}

trait SuperTrait {
    fn call_super(&self);
}

trait SubTrait: SuperTrait {
    fn call_sub(&self);
}

struct Baz;

impl SuperTrait for Baz {
    fn call_super(&self) {
        println!("call super");
        self.call_sub(); // so not java :D
    }
}

impl SubTrait for Baz {
    fn call_sub(&self) {
        println!("call sub");
        self.call_super();
    }
}
