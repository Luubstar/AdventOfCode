using commons;
class Dia5{
    public static string Execute(){
        string[] input = Commons.ReadLines("Dia5/dia5.txt");
        return "Primera parte: " + Part1(input) + "\nSegunda parte: " + Part2(input);
    }

    private static string Part1(string[] input){
        int nices = 0;
        foreach(string line in input){
            int numVocales = 0;
            bool dosSeguidas = false;
            char lastChar = ' ';
            bool invalid = false;

            foreach(char c in line){
                if(c == 'a' ||c == 'e' ||c =='i' || c=='o' || c=='u'){numVocales++;}

                string duo = lastChar.ToString() + c.ToString();
                if(duo == "ab" ||duo == "cb" || duo == "pq" || duo == "xy"){invalid = true; break;}
                if(lastChar == c){dosSeguidas = true;}
                lastChar = c;
            }

            if(!invalid && dosSeguidas && numVocales >= 3){nices++;}
        }
        return nices.ToString();
    }

    private static string Part2(string[] input){
        int nices = 0;

        foreach(string line in input){
            Dictionary<string, int> duos = new Dictionary<string, int>();
            string lastDuo = "";
            char lastChar = ' ';
            char prevChar = ' ';

            bool fst = false, snd = false;
            bool invalid = false;

            foreach(char c in line){
                if(duos.ContainsKey(lastChar.ToString() + c.ToString())){
                    duos[lastChar.ToString() + c.ToString()]++;
                }
                else{
                    duos[lastChar.ToString() + c.ToString()] = 1;
                }

                if(lastDuo == lastChar.ToString() + c.ToString()){invalid = true; break;}
                lastDuo = lastChar.ToString() + c.ToString();

                if(prevChar == c && c != lastChar){snd = true;}
                prevChar = lastChar;
                lastChar = c;
            }

            foreach (var kvp in duos)
            {
                if(kvp.Value >= 2){fst = true; break;}
            }
            
            if(fst && snd && !invalid){nices++;}
        }
        return nices.ToString();
    }

}