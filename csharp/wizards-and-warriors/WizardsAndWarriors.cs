abstract class Character
{
    public abstract int DamagePoints(Character target);

    public virtual bool Vulnerable() => false;

    public override string ToString() => $"Character is a {this.GetType()}";
}

class Warrior : Character
{
    public override int DamagePoints(Character target) => target.Vulnerable() ? 10 : 6;
}

class Wizard : Character
{
    private bool preparedSpell = false;

    public override int DamagePoints(Character target) => this.preparedSpell ? 12 : 3;

    public void PrepareSpell()
    {
        this.preparedSpell = true;
    }

    public override bool Vulnerable() => !this.preparedSpell;
}
