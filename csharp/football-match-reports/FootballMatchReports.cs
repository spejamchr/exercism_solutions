public static class PlayAnalyzer
{
    public static string AnalyzeOnField(int shirtNum) =>
        shirtNum switch
        {
            1 => "goalie",
            2 => "left back",
            3 or 4 => "center back",
            5 => "right back",
            6 or 7 or 8 => "midfielder",
            9 => "left wing",
            10 => "striker",
            11 => "right wing",
            _ => "UNKNOWN",
        };

    public static string AnalyzeOffField(object report) =>
        report switch
        {
            int count => $"There are {count} supporters at the match.",
            string announcement => announcement,
            Injury injury => $"Oh no! {injury.GetDescription()} Medics are on the field.",
            Incident incident => incident.GetDescription(),
            Manager manager => $"{manager.Name}{ManagerClubPart(manager)}",
            _ => "",
        };

    private static string ManagerClubPart(Manager manager) =>
        manager.Club == null ? "" : $" ({manager.Club})";
}
