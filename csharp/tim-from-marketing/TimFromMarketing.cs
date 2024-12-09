static class Badge
{
    public static string Print(int? id, string name, string? department) =>
        $"{IdPart(id)}{name} - {DepartmentName(department)}";

    private static string IdPart(int? id) => (id == null) ? "" : $"[{id}] - ";

    private static string DepartmentName(string? department) =>
        department?.ToUpperInvariant() ?? "OWNER";
}
