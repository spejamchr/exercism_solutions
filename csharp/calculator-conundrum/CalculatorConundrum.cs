using System;

public static class SimpleCalculator
{
    public static string Calculate(int operand1, int operand2, string operation)
    {
        int result;
        try
        {
            result = CalculateResult(operand1, operand2, operation);
        }
        catch (DivideByZeroException)
        {
            return "Division by zero is not allowed.";
        }
        return $"{operand1} {operation} {operand2} = {result}";
    }

    private static int CalculateResult(int operand1, int operand2, string operation)
    {
        return operation switch
        {
            "+" => operand1 + operand2,
            "*" => operand1 * operand2,
            "/" => operand1 / operand2,
            "" => throw new ArgumentException(),
            null => throw new ArgumentNullException(),
            _ => throw new ArgumentOutOfRangeException(),
        };
    }
}
