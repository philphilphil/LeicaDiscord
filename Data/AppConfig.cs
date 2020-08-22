using System;
using System.Collections.Generic;
using System.Text;

namespace KenR_LeicaBot.Data
{
    public class AppConfig
    {
        public AppConfig()
        {
        }

        public string API_token { get; set; }
        public ulong Camera_Role_Message_Id { get; set; }

        public IEnumerable<CameraRoleMaping> Camera_Role_Maping { get; set; }
        public ulong Camera_Role_Channel_Id { get; set; }
    }

    public class CameraRoleMaping
    {
        public string emoji { get; set; }
        public string role { get; set; }
    }
}
