@group(0)
@binding(0)
var<storage, read_write> monkey_initial_values: array<u32>; // this is used as both input and output for convenience
@group(0)
@binding(1)
var<storage, read_write> strategy_rewards: array<u32>; // this is used as both input and output for convenience
@group(0)
@binding(2)
var<uniform> len_monkeys: u32;

/*

        let mut nv = self.0 as u64;
        nv ^= self.0 as u64 * with as u64;
        nv %= Self::PRUNE_MOD as u64;
        MonkeySecret(nv as u32)
    }
    fn mix_div_prune(&self, with: u32) -> Self {
        let mut nv = self.0 as u64;
        nv ^= self.0 as u64 / with as u64;
        nv %= Self::PRUNE_MOD as u64;
        MonkeySecret(nv as u32)
    }

    fn next_secret(&self) -> Self {
        self.mix_mul_prune(64)
            .mix_div_prune(32)
            .mix_mul_prune(2048)
*/
fn next_secret(secret: u32) -> u32 {
    let MOD = 16777216u;
    var s = secret;
    s = s ^ (s * 64);
    s = s % MOD;
    s = s ^ (s / 32);
    s = s % MOD;
    s = s ^ (s * 2048);
    s = s % MOD;
    return s;
}

fn would_sell_for(initial_secret: u32, expect_a: i32, expect_b: i32, expect_c: i32, expect_d: i32) -> u32 {
    var prev = initial_secret;
    var secret = next_secret(initial_secret);
    var diff_a = i32(secret % 10) - i32(prev % 10);
    prev = secret;
    secret = next_secret(prev);
    var diff_b = i32(secret % 10) - i32(prev % 10);
    prev = secret;
    secret = next_secret(prev);
    var diff_c = i32(secret % 10) - i32(prev % 10);
    prev = secret;
    secret = next_secret(prev);
    var diff_d = i32(secret % 10) - i32(prev % 10);

    for (var j = 0u; j < (2000 - 4); j++) {
        // check for match
        if diff_a == expect_a && diff_b == expect_b && diff_c == expect_c && diff_d == expect_d {
            return secret % 10;
        }
        // Rotate once
        diff_a = diff_b;
        diff_b = diff_c;
        diff_c = diff_d;
        prev = secret;
        secret = next_secret(secret);
        diff_d = i32(secret % 10) - i32(prev % 10);
    }
    return 0u;
}

@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let d1 = i32(global_id[0] % 19) - 9;
    let d2 = i32((global_id[0] / 19) % 19) - 9;
    let d3 = i32(global_id[1] % 19) - 9;
    let d4 = i32((global_id[1] / 19) % 19) - 9;
    let index = (19u*19u)*global_id[0] + global_id[1];

    var partial = d1+d2;
    if partial < -9 || 9 < partial {
        strategy_rewards[index] = 2u;
        return;
    }
    partial = partial + d3;
    if partial < -9 || 9 < partial {
        strategy_rewards[index] = 2u;
        return;
    };
    partial = partial + d4;
    if partial < -9 || 9 < partial {
        strategy_rewards[index] = 2u;
        return;
    };
    var total = 0u;
    for(var i = 0u; i < len_monkeys; i++) {
        var initial_value = monkey_initial_values[i];
        total += would_sell_for(initial_value, d1, d2, d3, d4);
    };
    strategy_rewards[index] = total + 1;
}