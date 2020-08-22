using Discord;
using Discord.WebSocket;
using KenR_LeicaBot.Data;
using Microsoft.Extensions.Configuration;
using System;
using System.Collections.Generic;
using System.Text;
using System.Threading.Tasks;

namespace KenR_LeicaBot.Services
{
    public class RoleService
    {
        private readonly AppConfig _config;

        public RoleService(AppConfig config)
        {
            _config = config;
        }
        public Task ReactionAddedEvent(Cacheable<IUserMessage, ulong> msg, ISocketMessageChannel chan, SocketReaction react)
        {
            if (msg.Id != 746326906412204073) return Task.CompletedTask;


            throw new NotImplementedException();
        }

        internal Task ReactionRemovedEvent(Cacheable<IUserMessage, ulong> msg, ISocketMessageChannel chan, SocketReaction react)
        {
            throw new NotImplementedException();
        }
    }
}
