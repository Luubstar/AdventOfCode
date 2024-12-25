using System.Drawing;
using commons;
class Dia16{
    public static string Execute(){
        string[] input = Commons.ReadLines("Dia16/dia16.txt");
        string[] datos = Commons.ReadLines("Dia16/datos.txt");
        return "Primera parte: " + Part1(input, datos);
    }

    private static string Part1(string[] input, string[] datos){
        List<Abuela> abuelas = Abuela.CrearAbuelas(input);
        Abuela objetivo = new Abuela(datos);

        int i = 1;
        foreach(Abuela a in abuelas){
            if(a.EqualsP1(objetivo)){break;}
            i++;
        }

        int j = 1;
        foreach(Abuela a in abuelas){
            if(a.EqualsP2(objetivo)){break;}
            j++;
        }

        return i.ToString() + " | " + j.ToString();
    }

}

class Abuela{
    public int children = -1;
    public int cats = -1;
    public int samoyeds = -1;
    public int pomeranians = -1;
    public int akitas = -1;
    public int vizslas = -1;
    public int goldfish = -1;
    public int trees = -1;
    public int cars = -1;
    public int perfumes = -1;

    public Abuela(){}

    public Abuela(string[] d){
        foreach(string dato in d){
            AddPropery(dato.Split(": ")[0], int.Parse(dato.Split(": ")[1]));
        }
    }
    public static List<Abuela> CrearAbuelas(string[] input){
        List<Abuela> abuelas = new List<Abuela>();
        foreach(string linea in input){
            Abuela a = new Abuela();
            string datos = linea.Substring(linea.IndexOf(' ')+1);
            datos = datos.Substring(datos.IndexOf(' ')+1);
            foreach(string dato in datos.Split(", ")){
                string tag = dato.Split(": ")[0];
                int v = int.Parse(dato.Split(": ")[1]);
                a.AddPropery(tag, v);
            }
            abuelas.Add(a);
        }
        return abuelas;
    }

    public bool EqualsP1(Abuela o)
    {
        bool canBe = true;
        if(children != -1){canBe = o.children == children;}
        if(canBe && cats != -1){canBe = o.cats == cats;}
        if(canBe && samoyeds != -1){canBe = o.samoyeds == samoyeds;}
        if(canBe && pomeranians != -1){canBe = o.pomeranians == pomeranians;}
        if(canBe && akitas != -1){canBe = o.akitas == akitas;}
        if(canBe && vizslas != -1){canBe = o.vizslas == vizslas;}
        if(canBe && goldfish != -1){canBe = o.goldfish == goldfish;}
        if(canBe && trees != -1){canBe = o.trees == trees;}
        if(canBe && perfumes != -1){canBe = o.perfumes == perfumes;}
        return canBe;
    }

    public bool EqualsP2(Abuela o)
    {
        bool canBe = true;
        if(children != -1){canBe = o.children == children;}
        if(canBe && cats != -1){canBe = cats > o.cats;}
        if(canBe && samoyeds != -1){canBe = o.samoyeds == samoyeds;}
        if(canBe && pomeranians != -1){canBe = pomeranians < o.pomeranians;}
        if(canBe && akitas != -1){canBe = o.akitas == akitas;}
        if(canBe && vizslas != -1){canBe = o.vizslas == vizslas;}
        if(canBe && goldfish != -1){canBe = goldfish < o.goldfish;}
        if(canBe && trees != -1){canBe = trees > o.trees;}
        if(canBe && perfumes != -1){canBe = o.perfumes == perfumes;}
        return canBe;
    }

    public void AddPropery(string tag, int v){
        switch (tag){
            case "children":
                children = v;
                break;
            case "cats":
                cats = v;
                break;
            case "samoyeds":
                samoyeds = v;
                break;
            case "pomeranians":
                pomeranians = v;
                break;
            case "akitas":
                akitas = v;
                break;
            case "vizslas":
                vizslas = v;
                break;
            case "goldfish":
                goldfish = v;
                break;
            case "trees":
                trees = v;
                break;
            case "cars":
                cars = v;
                break;
            case "perfumes":
                perfumes = v;
                break;
        }
    }
}