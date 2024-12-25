using Newtonsoft.Json.Linq;
using commons;
class Dia12{
    public static string Execute(){
        string input = Commons.ReadInput("Dia12/dia12.txt");
        return "Primera parte: " + Part1(input) + "\nSegunda parte: " + Part2(input);
    }

    private static string Part1(string input){
        int i = 0;
        for(int j = 0; j < input.Length; j++){
            char c = input[j];
            if(c == '-' && Char.IsNumber(input[j+1])){
                string v = GetNumber(input, j+1);
                i -= int.Parse(v);
                j += v.Length + 1;
            }
            else if(Char.IsNumber(c)){
                string v = GetNumber(input, j);
                i += int.Parse(v);
                j += v.Length;
            }
        }
        return i.ToString();
    }
    private static string Part2(string input){
        JToken parsedJson = JToken.Parse(input);

        // Procesa el JSON para eliminar objetos con la palabra "red"
        JToken result = RemoveRedObjects(parsedJson);
        return Part1(result.ToString());
    }

    static JToken RemoveRedObjects(JToken token)
    {
        if (token.Type == JTokenType.Object)
        {
            JObject obj = (JObject)token;
            foreach (var property in obj.Properties().ToList())
            {
                if (property.Value.Type == JTokenType.String && property.Value.ToString() == "red")
                {
                    return null; // Si se encuentra la palabra "red" en algún valor, retorna null para eliminar este objeto
                }
                else
                {
                    obj[property.Name] = RemoveRedObjects(property.Value);
                }
            }
        }
        else if (token.Type == JTokenType.Array)
        {
            JArray array = (JArray)token;
            for (int i = array.Count - 1; i >= 0; i--)
            {
                array[i] = RemoveRedObjects(array[i]);
                if (array[i] == null)
                {
                    array.RemoveAt(i);
                }
            }
        }

        return token;
    }
    

    private static string GetNumber(string input, int pos){
        int i = pos;
        string res = "";
        while(Char.IsNumber(input[i])){
            res += "" + input[i];
            i++;
        }
        return res;
    }

    
}