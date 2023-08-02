# Proof of Induction

## What is Proof by Induction?

Proof by induction is a mathematical method used to prove that a statement is true for all natural numbers. It’s not enough to prove that a statement is true in one or more specific cases. We need to prove it is true for all cases.

## What Problem(s) Does Proof by Induction Solve?
If a statement is true for one case, proof by induction helps us prove it is true for all cases.

For our purposes, proof by induction will help us calculate the Big O of recursive algorithms.

## Reasoning

Inductive reasoning and deductive reasoning are two methods of reason in logic.

### Deductive reasoning is top-down.

We start with general properties that are true and from them determine the truth of a specific property.

The classic example is a syllogism:

1. All men are mortal.
2. Socrates is a man.
3. Therefore, Socrates is mortal.

### Inductive reasoning is bottom-up.

We start with specific properties, look for patterns, and make generalizations.

For example:
1. The sun has risen in the east every morning up until now.
2. The sun will also rise in the east tomorrow.

That’s not good enough! <br>
We skeptics need proof.

> A mathematical proof shows that “the stated assumptions logically guarantee the conclusion”.

<b> Despite its name, proof by induction is a method of deduction.</b>


Why?

We need to prove that the specific case is true for all cases, without exception. Inductive reasoning doesn’t guarantee this.

Unlike deductive reasoning, we don’t start with a general rule.

We start with the rule we want to prove and assume it is true and then use mathematics to prove it generally.

There are two steps to proof by induction:
- Base
- Induction


### Proof by Induction: Base
We first need to prove that our property holds for a natural number.

That’s generally 0 or 1.

### Proof by Induction: Induction
Once we establish a base case, we need to prove that property holds for the next natural number.

What’s the next natural number?

n + 1

#### How to Sum Consecutive Integers 1 to n
    P = n * (n + 1) / 2

We proved that our equation works when n = 1.

What if we don’t know n?

Let’s add another variable to the equation, k.

Let’s say that k is less than or equal to n.

    k <= n

We need to make a proposition. What is it?

    If P(k) is true, then P(k + 1) is also true.

If that’s true, then P(n) is true for all natural numbers.

