using commons;
class Dia14{
    public static string Execute(){
        string[] input = Commons.ReadLines("Dia14/dia14.txt");
        return "Primera parte: " + Part1(input) + "\nSegunda parte: " + Part2(input);
    }

    private static string Part1(string[] input){
        int max = 0;
        foreach(string line in input){
            string[] segmentos = line.Split(" ");
            Reno reno = new Reno(int.Parse(segmentos[3]), int.Parse(segmentos[6]), int.Parse(segmentos[13]));

            max = Math.Max(max, reno.Correr(2503));
        }

        return max.ToString();
    }

    private static string Part2(string[] input){
        int max = 0;
        List<Reno> renos = new List<Reno>();
        foreach(string line in input){
            string[] segmentos = line.Split(" ");
            Reno reno = new Reno(int.Parse(segmentos[3]), int.Parse(segmentos[6]), int.Parse(segmentos[13]));
            renos.Add(reno);
        }

        int tiempo = 2503;
        while(tiempo > 0){
            foreach(Reno r in renos){
                r.cuenta++;
                if(r.descansando){
                    if(r.cuenta >= r.descansoPostVuelo){
                        r.cuenta = 0;
                        r.descansando = false;
                    }
                }
                else{
                    r.camino += r.velocidadVuelo;
                    if(r.cuenta >= r.tiempoVuelo){
                        r.cuenta = 0;
                        r.descansando = true;
                    }
                }
            }

            List<Reno> renosGanadores = new List<Reno>();

            foreach(Reno r in renos){
                if (renosGanadores.Count == 0){renosGanadores.Add(r);}
                else if (r.camino > renosGanadores[0].camino){
                    renosGanadores.Clear();
                    renosGanadores.Add(r);
                }
                else if (r.camino == renosGanadores[0].camino){renosGanadores.Add(r);}
            }

            foreach(Reno r in renosGanadores){
                r.puntos++;
            }

            tiempo--;
        }

        foreach(Reno r in renos){
            max = Math.Max(max, r.puntos);
        }

        return max.ToString();
    }
}


class Reno{
    public int velocidadVuelo;
    public int tiempoVuelo;
    public int descansoPostVuelo;
    public bool descansando;
    public int cuenta = 0;
    public int camino = 0;

    public int puntos = 0;

    public Reno(int vV, int tDV, int dPV){
        velocidadVuelo = vV;
        tiempoVuelo = tDV;
        descansoPostVuelo = dPV;
    }

    public int Correr(int t){
        int c = (int) Math.Ceiling((double) t / (tiempoVuelo + descansoPostVuelo));
        return c * tiempoVuelo * velocidadVuelo;
    }
}