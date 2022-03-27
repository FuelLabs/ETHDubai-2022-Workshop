<img src="./images/solidity_logo.svg" width=200px alt="solidity" style="filter: invert(70%);"/>

Language for creating smart contracts:

<ul class="size">
    <li>Developed for early EVM</li>
    <li>Feels similar to Javascript</li>
    <li>Domain-specific for smart contracts</li>
</ul>

Note:

* Integrates contract storage as a language construct
* Allows for cross contract calls

---

<img src="./images/solidity_logo.svg" width=200px alt="solidity" style="filter: invert(70%);" /></h2>

but it has some issues...

<ul class="size">
    <li>Solidity docs don't tell you how to do contracts end-to-end</li>
    <li>Fragmented ecosystem + tooling</li>
    <li>Lacks many critical safety guarantees</li>
</ul>

---

<img src="./images/solidity_logo.svg" width=200px alt="solidity" style="filter: invert(70%);" /></h2>

wide classes of bugs:

<ul class="size">
    <li>Reentrancy issues</li>
    <li>Magic values used for errors</li>
    <li>"stack too deep" errors...</li>
</ul>

---

<div class="container">

<div class="col">
<img src="./images/angry_tweet3.png" height=150px />
</div>

<div class="col">
<img src="./images/angry_tweet.png" height=500px />
</div>

</div>

Note:

it kinda works but...

was designed without knowledge of blockchain smart contract requirements