using System;

class WeighingMachine
{
    public int Precision { get; }
    public double TareAdjustment { get; set; } = 5.0;

    public WeighingMachine(int precision)
    {
        Precision = precision;
    }

    private double _weight = 0;
    public double Weight
    {
        get { return _weight; }
        set
        {
            if (value < 0)
                throw new ArgumentOutOfRangeException();
            _weight = value;
        }
    }

    public string DisplayWeight
    {
        get { return $"{(Weight - TareAdjustment).ToString($"F{Precision}")} kg"; }
    }
}
