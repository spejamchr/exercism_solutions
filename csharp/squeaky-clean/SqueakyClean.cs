using System;
using System.Text;

public static class Identifier
{
    public static string Clean(string identifier)
    {
        if (identifier == "")
            return "";

        var builder = new StringBuilder();

        for (int i = 0; i < identifier.Length; i++)
        {
            var c = identifier[i];
            if (c == ' ')
                builder.Append('_');
            else if (Char.IsControl(c))
                builder.Append("CTRL");
            else if (c == '-')
            {
                i++;
                if (i < identifier.Length)
                    builder.Append(identifier[i].ToString().ToUpperInvariant());
            }
            else if (Char.IsLetter(c) && (c < 'α' || 'ω' < c))
                builder.Append(c);
        }

        return builder.ToString();
    }
}
