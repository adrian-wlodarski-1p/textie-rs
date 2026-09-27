text = []
inp = ''

try:
    while inp != '?exit':
        inp = input(': ')
        isp = inp.split()
        isp = isp if isp else ['']
        if inp == '?clear':
            text = []
        elif isp[0] == '?delete':
            isp[1] = int(isp[1])
            del text[isp[1]]
        elif isp[0] == '?edit':
            isp[1] = int(isp[1])
            text[isp[1]] = input()
        elif isp[0] == '?print':
            if len(isp) == 1:
                print('\n'.join(text))
            else:
                isp[1] = int(isp[1])
                print(text[isp[1]])
        elif isp[0] == '?save':
            with open(isp[1], mode='w') as f:
                f.write('\n'.join(text))
        elif isp[0] in ('?open','?load'):
            with open(isp[1], mode='r') as f:
                text = f.read().split('\n')
        elif inp.endswith('|'):
            text[-1] += inp
        else:
            text.append(inp)
        with open('backup.txt', 'w') as fb:
            fb.write('\n'.join(text))
except Exception as e:
    print(e)
    input()
