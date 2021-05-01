using System;
using System.Collections.Generic;

namespace LeicaD.Web.Data
{
    public class User
    {
        public string Username { get; set; }
        public List<string> Roles { get; set; }
    }
}
