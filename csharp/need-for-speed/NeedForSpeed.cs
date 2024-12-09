class RemoteControlCar
{
    private int battery = 100;
    private int speed;
    private int batteryDrain;

    public RemoteControlCar(int speed, int batteryDrain)
    {
        this.speed = speed;
        this.batteryDrain = batteryDrain;
    }

    public bool BatteryDrained() => this.battery < this.batteryDrain;

    public int DistanceDriven() => (100 - this.battery) / this.batteryDrain * this.speed;

    public void Drive()
    {
        if (!this.BatteryDrained())
            this.battery -= this.batteryDrain;
    }

    public static RemoteControlCar Nitro() => new(50, 4);
}

class RaceTrack
{
    private int distance;

    public RaceTrack(int distance)
    {
        this.distance = distance;
    }

    public bool TryFinishTrack(RemoteControlCar car)
    {
        while (car.DistanceDriven() < this.distance && !car.BatteryDrained())
            car.Drive();

        return car.DistanceDriven() >= this.distance;
    }
}
