using Discord;
using Discord.Commands;
using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace KenR_LeicaBot.Services
{
    public class ChannelPurgeService
    {
        public async Task PurgeAsync(SocketCommandContext context)
        {
            var channel = "bot"; 

            var msg = context.Message;
            var gallery = context.Guild.TextChannels.Where(x => x.Name == channel).FirstOrDefault();
            var messages = await gallery.GetMessagesAsync(10000).FlattenAsync(); 
            int i = 0;
            int i2 = 0;

            // var messages = await msg.Channel.GetMessagesAsync(100);
            foreach (IUserMessage message in messages)
            {
                //Skip messages with Attachments and special links
                if (message.Attachments.Count > 0 || ContentInAllowList(message.Content))
                {
                    i2++;
                    continue;
                }

                i++;
                await message.DeleteAsync(); //rate limit exception if too many messages!!!
            }

            await context.Channel.SendMessageAsync($"Deleted {i} Messages and skipped {i2} in channel {channel}");
        }

        private bool ContentInAllowList(string content)
        {
            List<string> allowedStrings = new List<string>(); //TODO: add this list to config/database
            allowedStrings.Add("instagram.com");
            allowedStrings.Add("imgur.com");

            foreach (var item in allowedStrings)
            {
                if (content.Contains(item)) return true;
            }

            return false;
        }
    }
}
