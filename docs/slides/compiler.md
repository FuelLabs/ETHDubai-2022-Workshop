### "friendly compiler" ?

---

<ul class="size">
    <li>Compiler messages are instructive and helpful</li>
    <li>Warnings and errors are specific to smart contracts</li>
    <li>Compiler detects smart contract bugs (e.g. reentrancy)</li>
</li>

---

<p class="size">
Sway code:
</p>

<img src="./images/sway_match_code.png" height=250px />

<p class="size">
...btw Sway has match expressions 😊
</p>

---

<p class="size">
Sway compiler:
</p>

<img src="./images/sway_match_error.png" height=250px />

Note:

<p class="size">
Solidity:
</p>

<img src="./images/solidity_switch.png" height=180px />

---

<p class="size">
Sway code:
</p>

<img src="./images/sway_decrement_counter.png" height=325px />

<p class="size">
Sway compiler:
</p>

<img src="./images/sway_cei_error.png" height=125px />

Note:

<p class="size">
Solidity:
</p>

<img src="./images/solidity_reentrancy_bug.png" height=50px />

<!--
Note: 
imagine you have this generic function:

```rust
fn two_of_the_same<T>(a: T, b: T) -> T {
    b
}

fn main() {
    let foo = two_of_the_same(1, 2);
    let bar = two_of_the_same(1, false);
}
```

<p class="size">
...btw Sway has generics 😊
</p>

<img src="./images/solidity_generics.png" height=180px />

then the Sway compiler would tell you this:
<img src="./images/sway_generics.png" height=200px />
-->