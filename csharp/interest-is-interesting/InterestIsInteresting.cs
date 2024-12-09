using System;

static class SavingsAccount
{
    public static float InterestRate(decimal balance) =>
        balance switch
        {
            < 0 => 3.213f,
            < 1000 => 0.5f,
            < 5000 => 1.621f,
            >= 5000 => 2.475f,
        };

    public static decimal Interest(decimal balance) =>
        balance * (decimal)InterestRate(balance) / 100;

    public static decimal AnnualBalanceUpdate(decimal balance) => balance + Interest(balance);

    public static int YearsBeforeDesiredBalance(decimal balance, decimal targetBalance)
    {
        if (balance <= 0)
            throw new ArgumentOutOfRangeException("Please provide a positive balance");

        // Can't easily use a log because the interest rate changes with the balance, so use a loop.
        var years = 0;
        while (balance < targetBalance)
        {
            years++;
            balance = AnnualBalanceUpdate(balance);
        }

        return years;
    }
}
