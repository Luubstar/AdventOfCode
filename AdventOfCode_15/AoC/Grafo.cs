namespace commons{

    class Grafo 
    {  
        private Dictionary<string, Dictionary<string, int>> _grafo = new Dictionary<string, Dictionary<string, int>>();

        public Grafo(){_grafo = new Dictionary<string, Dictionary<string, int>>();}

        public virtual void AddConnection(string o, string d, int c){
            if(!_grafo.ContainsKey(o)){_grafo[o] = new Dictionary<string, int>();}
            Dictionary<string, int> res = _grafo[o];
            res[d] = c;
            _grafo[o] = res;
        }

        public virtual Dictionary<string, int> Adjacent(string o){
            return _grafo[o];
        }

        public virtual string[] GetNodes(){
            return _grafo.Keys.ToArray();
        }

        public virtual Dictionary<string, Dictionary<string, int>> GetGraph(){return _grafo;}

        public override string ToString(){
            string res = Commons.GREEN + "Mostrando nodos y conexiones:\n";
            foreach(string v in _grafo.Keys){
                res += "Conexiones de " + v + "\n";
                res += "------------------\n";
                foreach(string c in _grafo[v].Keys){
                    res += c + " -> " + _grafo[v][c] + "\n";
                } 
                res += "------------------\n";
            }

            return res + Commons.NORMAL;
        }
    }
}