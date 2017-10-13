/*
 * Copyright (C) 2016 Lucien XU <sfietkonstantin@free.fr>
 *
 * You may use this file under the terms of the BSD license as follows:
 *
 * "Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *   * Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 *   * Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in
 *     the documentation and/or other materials provided with the
 *     distribution.
 *   * The names of its contributors may not be used to endorse or promote
 *     products derived from this software without specific prior written
 *     permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE."
 */

#ifndef DEQBASELISTMODEL_H
#define DEQBASELISTMODEL_H

#include <QAbstractListModel>
#include <deque>
#include <map>
#include <memory>
#include <vector>

class DEQBaseListModel : public QAbstractListModel
{
    Q_OBJECT
    Q_PROPERTY(int count READ count NOTIFY countChanged)
public:
    using Data = std::map<int, QVariant>;
    explicit DEQBaseListModel(std::map<int, QByteArray> &&roleNames, QObject *parent = nullptr);
    int count() const;
    bool empty() const;
    QHash<int, QByteArray> roleNames() const override;
    int rowCount(const QModelIndex &index = QModelIndex{}) const override;
    QVariant data(const QModelIndex &index, int role = Qt::DisplayRole) const override;
    Data get(int row) const;
    bool set(int row, Data &&value);
    bool insert(int row, std::vector<Data> &&values);
    bool remove(int row, int count);
    Data fromKeyValue(std::map<QString, QVariant> &&value) const;
    std::map<QString, QVariant> toKeyValue(Data &&data) const;
signals:
    void countChanged();

private:
    Data filterCompatible(Data &&data) const;
    std::vector<Data> filterCompatible(std::vector<Data> &&data) const;
    QHash<int, QByteArray> m_roleNames{};
    std::deque<Data> m_data{};
};

#endif // DEQBASELISTMODEL_H
