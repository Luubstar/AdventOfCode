using System.Diagnostics.CodeAnalysis;
using commons;
class Dia21{    
    public static string Execute(){
        string[] input = Commons.ReadLines("Dia21/dia21.txt");
        (List<Item>, List<Item>, List<Item>) items = Parse(Commons.ReadLines("Dia21/shop.txt"));
        return "Primera parte: " + Part1(input, items) + "\nSegunda parte: " + Part2(input, items);
    }

    private static string Part1(string[] input, (List<Item> w, List<Item> a, List<Item> r) items){
        int myhp = 100;
        
        int hp = int.Parse(input[0].Split(": ")[1]);
        int dm = int.Parse(input[1].Split(": ")[1]);
        int ar = int.Parse(input[2].Split(": ")[1]);

        items.w = items.w.OrderBy(p => p.cost).ToList<Item>();
        items.a = items.a.OrderBy(p => p.cost).ToList<Item>();
        items.r = items.r.OrderBy(p => p.cost).ToList<Item>();

        int min = int.MaxValue;

        foreach(Item weapon in items.w){
            foreach(Item armor in items.a){
                foreach(Item ring1 in items.r){
                    foreach(Item ring2 in items.r){
                        if(!ring1.Equals(ring2)){
                            int totalD = weapon.damage + armor.damage + ring1.damage + ring2.damage;
                            int totalAr = weapon.armor + armor.armor + ring1.armor + ring2.armor;
                            int dañoMonstruo = Math.Max(dm - totalAr,1); 
                            int turnosMatar = hp/(totalD-ar);
                            int turnosMorir = myhp/dañoMonstruo;

                            if(turnosMorir > turnosMatar){
                                min = Math.Min(min, weapon.cost + armor.cost + ring1.cost + ring2.cost);
                            }
                        }
                    }
                }
            }
        }

        return min.ToString();
    }

    private static string Part2(string[] input, (List<Item> w, List<Item> a, List<Item> r) items){
        int myhp = 100;
        
        int hp = int.Parse(input[0].Split(": ")[1]);
        int dm = int.Parse(input[1].Split(": ")[1]);
        int ar = int.Parse(input[2].Split(": ")[1]);

        items.w = items.w.OrderBy(p => p.cost).ToList<Item>();
        items.a = items.a.OrderBy(p => p.cost).ToList<Item>();
        items.r = items.r.OrderBy(p => p.cost).ToList<Item>();

        int max = 0;

        foreach(Item weapon in items.w){
            foreach(Item armor in items.a){
                foreach(Item ring1 in items.r){
                    foreach(Item ring2 in items.r){
                        if(!ring1.Equals(ring2)){
                            int totalD = weapon.damage + armor.damage + ring1.damage + ring2.damage;
                            int totalAr = weapon.armor + armor.armor + ring1.armor + ring2.armor;
                            int dañoMonstruo = Math.Max(dm - totalAr,1); 
                            int turnosMatar = hp/(totalD-ar);
                            int turnosMorir = myhp/dañoMonstruo;

                            if(turnosMorir <= turnosMatar){
                                max = Math.Max(max, weapon.cost + armor.cost + ring1.cost + ring2.cost);
                            }
                        }
                    }
                }
            }
        }

        return max.ToString();
    }
    private static (List<Item>, List<Item>, List<Item>) Parse(string[] shop){
        List<Item> armas = new List<Item>();
        List<Item> armaduras  = [Item.Empty()];
        List<Item> anillos = [Item.Empty(),Item.Empty()];

        int i = 0;
        while(shop[i] != "|"){
            string[] props = shop[i].Split(",");
            armas.Add(new Item(int.Parse(props[1]),int.Parse(props[2]),int.Parse(props[3])));
            i++;
        }
        i++;
        while(shop[i] != "|"){
            string[] props = shop[i].Split(",");
            armaduras.Add(new Item(int.Parse(props[1]),int.Parse(props[2]),int.Parse(props[3])));
            i++;
        }
        i++;
        while(i < shop.Length){
            string[] props = shop[i].Split(",");
            anillos.Add(new Item(int.Parse(props[1]),int.Parse(props[2]),int.Parse(props[3])));
            i++;
        }

        return (armas, armaduras, anillos);
    }
private struct Item : IEquatable<Item>{
    public readonly int cost;
    public readonly int armor;
    public readonly int damage;

    public Item(int c, int d, int a){
        cost = c;
        armor = a;
        damage = d;
    }

    public static Item Empty(){
        return new Item(0,0,0);
    }
    public bool Equals(Item other)
    {
        return other.cost == cost && other.armor == armor && other.damage == damage;
    }
}
}

