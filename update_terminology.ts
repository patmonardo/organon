import * as fs from 'fs';
import * as path from 'path';

const baseDir = path.resolve('./reality/src/logos/Reference/yoga-sutras');

function walkDir(dir: string, callback: (filepath: string) => void) {
  fs.readdirSync(dir).forEach(f => {
    let dirPath = path.join(dir, f);
    let isDirectory = fs.statSync(dirPath).isDirectory();
    isDirectory ? walkDir(dirPath, callback) : callback(dirPath);
  });
}

walkDir(baseDir, function(filePath) {
  if (filePath.endsWith('.md')) {
    let content = fs.readFileSync(filePath, 'utf8');
    let updated = content.replace(/Kaivalya\/Vibhuti padas/g, 'Dharma-pada (Chapter 4)');
    updated = updated.replace(/Kaivalya Pada/g, 'Dharma-pada');
    updated = updated.replace(/Kaivalya/g, 'Dharma-pada');
    
    if (content !== updated) {
      fs.writeFileSync(filePath, updated);
      console.log('Updated: ' + filePath);
    }
  }
});
