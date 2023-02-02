
fn main() {
    //region 14.泛型
    // 泛型(generic)是关于泛化类型和函数功能，以扩大其适用范围的话题。
    // 泛型极大地减少了代码的重复，但它自身的语法要求细心，也就是说，
    // 采用泛型意味着仔细地指定泛型具体化时，什么的具体类型是合法的。
    // 泛型最简单和常用的用法是用于类型参数。
    // 定义泛型类型或泛型函数时，会用到 <A> 或者 <T> 之类的作为类型的代号，
    // 就像函数的形参一样。在使用时，为把 <A>、<T> 具体化，我们会把类型说明像实参
    // 一样使用，就像是 <i32> 这样。
    // 泛型的类型参数是使用尖括号和大驼峰命名的名称来指定
    // 泛型类型参数一般用 <T> 来表示。
    // 例如定义一个名为 foo 的泛型函数，它可接受类型为 T 的任何参数 arg：
    // fn foo<T>(arg: T) {...}
    // 因为这里使用了泛型类型参数 <T> ，所以这里的 (arg: T) 中的 T 就是泛型类型
    // 即使 T 在之前被定义为 struct ，这里的 T 仍然代表泛型。

    // 一个具体的类型 `A`
    struct A;

    // 在定义类型 `Single` 时，第一次使用类型 `A` 之前没有写 `<A>`。
    // 因此，`Single` 是个具体类型，`A` 取上面的定义。
    struct Single(A);

    // 此处的 `<T>` 在第一次使用 `T` 前出现，所以 `SingleGen` 是一个泛型类型。
    // 因为 `T` 是泛型的，所以它可以是任何类型，包括在上面定义的具体类型 `A`。
    struct SingleGen<T>(T);
    // `Single` 为具体类型，并且显式地使用类型 `A`。
    let _s = Single(A);
    // 创建一个 `SingleGen<char>` 类型的变量 `_char`，并令其值为
    // `SingleGen('a')`，这里的 `SingleGen` 的类型参数是显式指定的。
    let _char: SingleGen<char> = SingleGen('a');

    // `SingleGen` 的类型参数也可以隐式地指定
    let _t = SingleGen(A);              // 使用在上面定义的 `A`
    let _i32 = SingleGen(6);            // 使用 `i32` 类型
    let _char = SingleGen('a');         // 使用 `char`
    //endregion

    //region 14.1.函数
    // 在使用类型 T 前给出 <T> ，那么 T 就变成了泛型。
    // 调用泛型函数有时需要显式地指明类型参量。 这可能是因为调用了返回类型是泛型的
    // 函数，或者编译器有足够的信息来推断类型参数。
    // 调用函数时，使用显式指定的类型参数会像是这样： fun::<A, B, ...>()。
    struct A1;
    struct S(A1);
    struct SGen<T>(T);
    // 定义一个函数 `reg_fn`，接受一个 `S` 类型的参数 `_s`。
    // 因为没有 `<T>` 这样的泛型类型参数，所以这不是泛型函数。
    fn reg_fn(_s: S) {}

    // 定义一个函数 `gen_spec_t`，接受一个 `SGen<A>` 类型的参数 `_s`。
    // `SGen<>`显式地接受了类型参数 `A`，且在 `gen_spec_t`中，`A` 没有被用作
    // 泛型类型的参数，所以函数不是泛型的。
    fn gen_spec_t(_s: SGen<A1>) {}

    // 定义一个函数 `gen_spec_i32`，接受一个 `SGen<i32>` 类型的参数 `_s`。
    // `SGen<>`显式地接受了类型参数 `i32`，而 `i32` 是一个具体类型。
    // 由于 `i32` 不是一个泛型类型，所以函数不是泛型的。
    fn gen_spec_i32(_s: SGen<i32>) {}


    // 定义一个函数 `generic`，接受一个 `SGen<T>` 类型的参数 `_s`。
    // `SGen<T>` 之前有 `<T>` ，所以这个函数是关于 `T` 的泛型函数。
    fn generic<T>(_s: SGen<T>) {}

    reg_fn(S(A1));
    gen_spec_t(SGen(A1));
    gen_spec_i32(SGen(5));

    // 为 `generic()` 显式地指定类型参数 `char`。
    generic::<char>(SGen('a'));

    // 为 `generic()` 隐式地指定类型参数 `char`。
    generic(SGen('c'));
    //endregion

    //region 14.2.实现
    println!("\n\n=====14.2.实现=====");
    struct B;
    struct GenericVal<T>(T,);
    impl GenericVal<f32> {}
    impl GenericVal<B> {}
    impl <T> GenericVal<T> {}

    struct Val {
        val: f64
    }
    struct GenVal<T> {
        gen_val: T
    }
    impl Val {
        fn value(&self) -> &f64 { &self.val }
    }
    impl <T> GenVal<T> {
        fn value(&self) -> &T {&self.gen_val }
    }
    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3i32 };
    println!("x is: {}, y is: {}", x.value(), y.value());
    //endregion

    //region 14.3.trait
    println!("\n\n=====14.3.trait=====");


    //endregion
}
