def nthFibonacci(n : int) -> int:
        mod = 10**9 + 7
        dp = [0] * (n + 5)
        dp[0] = 0
        dp[1] = 1

        for i in range(2, n + 1):
            dp[i] = (dp[i - 1] % mod + dp[i - 2] % mod) % mod

        return dp[n]