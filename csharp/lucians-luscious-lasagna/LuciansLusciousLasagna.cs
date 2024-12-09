class Lasagna
{
    public int ExpectedMinutesInOven() => 40;

    public int RemainingMinutesInOven(int current_time) =>
        this.ExpectedMinutesInOven() - current_time;

    public int PreparationTimeInMinutes(int layers) => layers * 2;

    public int ElapsedTimeInMinutes(int layers, int current_time) =>
        this.PreparationTimeInMinutes(layers) + current_time;
}
