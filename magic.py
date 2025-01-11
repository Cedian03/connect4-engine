INPUT_PATH = "./magic.rs"
OUTPUT_PATH = "./src/magic.rs"

IMPL = """\
impl Bar for {impl_type} {{
    type Qux = {qux_type};
}}\
"""


def foo(w: int, h: int) -> int:
    return w * (h + 1)


def bar(n: int) -> str:
    if n <= 8:
        return "u8"
    elif n <= 16:
        return "u16"
    elif n <= 32:
        return "u32"
    elif n <= 64:
        return "u64"
    elif n <= 128:
        return "u128"
    else:
        raise ValueError(f"No integer type big enough to fit {n} bytes")


def main():
    with open(OUTPUT_PATH, "w") as out:
        with open(INPUT_PATH, "r") as inp:
            print(inp.read(), file=out)

        for w in range(4, 16):
            for h in range(4, 16):
                if (n := foo(w, h)) <= 128:
                    impl_type = "Foo<{w}, {h}>".format(w=w, h=h)
                    qux_type = bar(n)

                    impl = IMPL.format(impl_type=impl_type, qux_type=qux_type)

                    print(impl, file=out)
                    print(file=out)


if __name__ == "__main__":
    main()
