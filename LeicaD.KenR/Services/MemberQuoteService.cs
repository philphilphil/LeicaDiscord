using Discord;
using Discord.WebSocket;
using KenR_LeicaBot.Data;
using Microsoft.Extensions.Configuration;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading;
using System.Threading.Tasks;

namespace KenR_LeicaBot.Services
{
    public class MemberQuoteService
    {
        private readonly AppConfig _config;
        private readonly DiscordSocketClient _discord;

        public MemberQuoteService(AppConfig config, DiscordSocketClient discord)
        {
            _config = config;
            _discord = discord;
        }
        public Task ReactionAddedEvent(Cacheable<IUserMessage, ulong> msg, ISocketMessageChannel chan, SocketReaction react)
        {
            if (msg.Id != _config.Camera_Role_Message_Id) return Task.CompletedTask;
            
            return Task.CompletedTask;
        }

        private void PostToQuoteChannel(string user, string message) {
            
        }
    }
}
