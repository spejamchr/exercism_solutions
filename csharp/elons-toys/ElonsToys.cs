class RemoteControlCar
{
    private int _batteryPercentage = 100;

    private int _distanceDriven() => (100 - this._batteryPercentage) * 20;

    public static RemoteControlCar Buy() => new();

    public string DistanceDisplay() => $"Driven {this._distanceDriven()} meters";

    public string BatteryDisplay() =>
        this._batteryPercentage == 0 ? "Battery empty" : $"Battery at {this._batteryPercentage}%";

    public void Drive()
    {
        if (this._batteryPercentage > 0)
            this._batteryPercentage--;
    }
}
