using System;

public static class LogAnalysis
{
    public static string SubstringAfter(this string str, string sub) => str.Split(sub, 2)[1];

    public static string SubstringBetween(this string str, string pre, string suf) {
      var startIndex = str.IndexOf(pre) + pre.Length;
      return str.Substring(startIndex, str.IndexOf(suf) - startIndex);
    }

    public static string Message(this string str) => str.SubstringAfter(": ");

    public static string LogLevel(this string str) => str.SubstringBetween("[", "]");
}
