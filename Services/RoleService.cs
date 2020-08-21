using Discord;
using Discord.WebSocket;
using System;
using System.Collections.Generic;
using System.Text;
using System.Threading.Tasks;

namespace KenR_LeicaBot.Services
{
    public static class RoleService
    {
        public static Task ReactionAddedEvent(Cacheable<IUserMessage, ulong> msg, ISocketMessageChannel chan, SocketReaction react)
        {
            if (msg.Id != 746326906412204073) return Task.CompletedTask;


            throw new NotImplementedException();
        }

        internal static Task ReactionRemovedEvent(Cacheable<IUserMessage, ulong> msg, ISocketMessageChannel chan, SocketReaction react)
        {
            throw new NotImplementedException();
        }
    }
}
