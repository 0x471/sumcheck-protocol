# Sumcheck Protocol Example

In this document, I provide a more in-depth explanation of Matteo's example from his blog post explaining the Sumcheck Protocol.

[Matteo's Blog Post on the Sumcheck Protocol](https://mtteo.dev/posts/understanding-sumcheck-protocol/understanding-sumcheck-protocol/#example)

---

We want to prove $H = 14$ with the following function:

$$
g(x_0, x_1, x_2) = 2 \cdot x_0^3 + x_1 + x_0 \cdot x_2
$$

This sum adds up $g(x_0, x_1, x_2)$ for all $2^3 = 8$ combinations of $x_0, x_1, x_2$. Instead of calculating all values, we use the Sumcheck Protocol.

---

## Step 1 - First Round of the Protocol

The prover sends $g_0(x_0)$:

$$
g_0(x_0) = 8x_0^3 + 2x_0 + 2
$$

The verifier checks $g_0(0) + g_0(1) = H$:

$$
g_0(0) = 8(0)^3 + 2(0) + 2 = 2
$$

$$
g_0(1) = 8(1)^3 + 2(1) + 2 = 8 + 2 + 2 = 12
$$

So:

$$
g_0(0) + g_0(1) = 2 + 12 = 14 = H
$$

The verifier confirms this step is valid and samples a random $r_0 = 12$ for the next round.

---

## Step 2 - Second Round of the Protocol

The prover sends $g_1(x_1)$:

$$
g_1(x_1) = 6924 + 2 \cdot x_1
$$

The verifier checks $g_1(0) + g_1(1) = g_0(r_0)$:

$$
g_1(0) = 6924 + 2(0) = 6924
$$

$$
g_1(1) = 6924 + 2(1) = 6926
$$

So:

$$
g_1(0) + g_1(1) = 6924 + 6926 = 13850 = g_0(12)
$$

The verifier confirms this step is valid and samples a random $r_1 = 5$ for the next round.

---

## Step 3 - Third Round of the Protocol

The prover sends $g_2(x_2)$:

$$
g_2(x_2) = 3461 + 12 \cdot x_2
$$

The verifier checks $g_2(0) + g_2(1) = g_1(r_1)$:

$$
g_2(0) = 3461 + 12(0) = 3461
$$

$$
g_2(1) = 3461 + 12(1) = 3473
$$

So:

$$
g_2(0) + g_2(1) = 3461 + 3473 = 6934 = g_1(5)
$$

The verifier confirms this step is valid and samples a random $r_2 = 2$ for the final step.

---

## Step 4 - Final Check

The verifier evaluates $g(r_0, r_1, r_2) = g(12, 5, 2)$ and confirms:

$$
g_2(r_2) = g(12, 5, 2)
$$

If this is true, the protocol is valid.
