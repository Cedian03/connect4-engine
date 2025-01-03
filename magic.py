import math

HEAD = """\
// No magic; just brute force.

pub struct MagicStruct<const N: usize>;

pub trait MagicTrait {
    type MagicType;
}
"""

IMPL = """\
impl MagicTrait for MagicStruct<{n}> {{
    type MagicType = {ty};
}}
"""

def next_power_of_two(n: int) -> int:
    if n <= 1:
        return 0

    return 1 << int(math.log2(n) + 1)

def main():
    print(HEAD)
    
    for n in range(128):
        print(IMPL.format(n=n+1, ty=f"u{max(next_power_of_two(n), 8)}"))
        

if __name__ == "__main__":
    main()