OUTPUT_PATH = "./src/magic/impls.rs"

HEAD = """\
use crate::board::Board;

use super::AsBitBoard;\
"""

IMPL = """
impl AsBitBoard for Board<{w}, {h}> {{
    type BitMask = {mask_type};
}}\
"""


def bits_needed(w: int, h: int) -> int:
    return w * (h + 1)


def uint_type(n: int) -> str:
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
    with open(OUTPUT_PATH, "w") as file:
        print(HEAD, file=file)

        w = h = 1
        while bits_needed(w, h) <= 128:
            while (w >= 4 or h >= 4) and (n := bits_needed(w, h)) <= 128:
                mask_type = uint_type(n)

                impl = IMPL.format(w=w, h=h, mask_type=mask_type)

                print(impl, file=file)

                w += 1

            w = 1
            h += 1


if __name__ == "__main__":
    main()
