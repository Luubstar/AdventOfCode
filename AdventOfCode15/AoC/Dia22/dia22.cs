using commons;
class Dia22{    
    public static int min = int.MaxValue;
    public static string Execute(){
        string[] input = Commons.ReadLines("Dia22/dia22.txt");
        List<(string, int)> hechizos = new List<(string, int)>
            {
                ("Missile", 53),
                ("Poison", 173),
                ("Shield", 113),
                ("Recharge", 229),
                ("Drain", 73)
            };

        return "Primera parte: " + Part1(input, hechizos) + "\nSegunda parte: " + Part2(input, hechizos);
    }

    private static string Part1(string[] input, List<(string, int)> hechizos){
        min = int.MaxValue;
        int myhp = 50;
        int mana = 500;
        
        int hp = int.Parse(input[0].Split(": ")[1]);
        int dm = int.Parse(input[1].Split(": ")[1]);

        int v = Resolver(new GameState(myhp, mana, hp, dm), hechizos, false);

        return v.ToString();
    }

    private static string Part2(string[] input, List<(string, int)> hechizos){
        min = int.MaxValue;
        int myhp = 50;
        int mana = 500;
        
        int hp = int.Parse(input[0].Split(": ")[1]);
        int dm = int.Parse(input[1].Split(": ")[1]);

        int v = Resolver(new GameState(myhp, mana, hp, dm), hechizos, true);

        return v.ToString();
    }

    public static int Resolver(GameState g, List<(string, int)> hechizos, bool part2){
        Resolver(g,hechizos,true, 0, part2);
        return min;
    }

    private static void Resolver(GameState g, List<(string, int)> hechizos, bool jugador, int v, bool part2){
        if(part2 && jugador){
            g.vida_jugador -= 1;
            if(g.vida_jugador <= 0){return;}
        }

        g.Efectos();
        if(g.vida_jefe <= 0){min = Math.Min(v, min);}
        
        if(jugador){
            foreach((string n, int c) hechizo in hechizos){
                if(g.mana_jugador - hechizo.c > 0){
                    GameState ng = g.Clone();
                    ng.mana_jugador -= hechizo.c;
                    if (v + hechizo.c < min){
                        bool can = ng.Hechizo(hechizo.n);
                        if(can && ng.vida_jefe <= 0){min = Math.Min(v+hechizo.c, min);}
                        else if(can) {Resolver(ng,hechizos,!jugador, v + hechizo.c, part2);}
                    }
                }
            }
        }
        else{
            g.vida_jugador -= Math.Max(1,g.daño_jefe - g.armadura_jugador);
            if(g.vida_jugador > 0){Resolver(g,hechizos,!jugador,v, part2);}
        }
    }

    public struct GameState{
        public int vida_jugador;
        public int mana_jugador;
        public int armadura_jugador;
        public int vida_jefe;
        public int daño_jefe;
        public List<(string, int)> efectos;

        public GameState(int vj, int mj, int hp, int dm){
            vida_jugador = vj;
            mana_jugador = mj;
            vida_jefe = hp;
            daño_jefe = dm;
            efectos = new List<(string n, int c)>();
        }

        public GameState Clone(){
            GameState g = new GameState(vida_jugador, mana_jugador, vida_jefe, daño_jefe)
            {
                efectos = new List<(string, int)>(efectos)
            };
            return g;
        }

        public bool Hechizo(string hechizo){
            switch(hechizo){
                case "Missile":
                    vida_jefe -= 4;
                    return true;
                case "Drain":
                    vida_jefe -= 2;
                    vida_jugador += 2;
                    return true;
                case "Shield":
                    if(efectos.Any(t => t.Item1 == "Shield")){return false;}
                    efectos.Add(("Shield", 6));
                    return true;
                case "Poison":
                    if(efectos.Any(t => t.Item1 == "Poison")){return false;}
                    efectos.Add(("Poison", 6));
                    return true;
                case "Recharge":
                    if(efectos.Any(t => t.Item1 == "Recharge")){return false;}
                    efectos.Add(("Recharge", 5));
                    return true;

            }
            return false;
        }

        public void Efectos(){
            List<(string,int)> e = new List<(string, int)>();
            armadura_jugador = 0;

            foreach((string n, int t) efecto in efectos){
                if(efecto.t - 1 > 0){e.Add((efecto.n, efecto.t-1));} 
                if(efecto.t >= 0){
                    switch(efecto.n){
                        case "Shield":
                            armadura_jugador = 7;
                            break;
                        case "Poison":
                            vida_jefe -= 3;
                            break;
                        case "Recharge":
                            mana_jugador += 101;
                            break;
                    }}
            }
            efectos = e;
        }
    }
}